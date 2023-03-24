use std::{cell::RefCell, rc::Rc};

type InstanceRefOf<T: ?Sized> = Rc<RefCell<T>>;

struct ConcreteUserRepository {
    users: Vec<User>,
}

type ConcreteUserRepositoryInstance = InstanceRefOf<ConcreteUserRepository>;

struct User {
    email: String,
    username: String,
}

trait Print {
    fn print(&self);
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            username: self.username.clone(),
            email: self.email.clone(),
        }
    }
}

impl Print for User {
    fn print(&self) {
        println!("email: {}, username: {}", self.email, self.username);
    }
}

impl User {
    fn new(email: String, username: String) -> User {
        User { email, username }
    }
}

trait UserRepository {
    fn save(&mut self, user: User);
    fn find_by_email(&self, email: String) -> Option<User>;
    fn get_all(&self) -> Vec<User>;
}

impl UserRepository for ConcreteUserRepository {
    fn save(&mut self, user: User) {
        let possible_user = (&self.users)
            .into_iter()
            .find(|user_iter| user_iter.email.eq(&user.email));
        if possible_user.is_some() {
            panic!("User already exist");
        }
        self.users.push(user);
    }

    fn find_by_email(&self, email: String) -> Option<User> {
        self.users
            .to_owned()
            .into_iter()
            .find(|user| user.email.eq(&email))
    }

    fn get_all(&self) -> Vec<User> {
        self.users.to_owned().clone()
    }
}

impl ConcreteUserRepository {
    fn new() -> ConcreteUserRepository {
        ConcreteUserRepository {
            users: vec![],
        }
    }
}

trait ApplicationService<T, R> {
    fn execute(&mut self, data: T) -> R;
}

struct RegisterUserApplicatioService {
    user_repo: InstanceRefOf<dyn UserRepository>,
}

struct UserData {
    email: String,
    username: String,
}

impl RegisterUserApplicatioService {
    fn new(user_repo: InstanceRefOf<dyn UserRepository>) -> RegisterUserApplicatioService {
        RegisterUserApplicatioService { user_repo }
    }
}

impl ApplicationService<UserData, Result<bool, &'static str>> for RegisterUserApplicatioService {
    fn execute(&mut self, data: UserData) -> Result<bool, &'static str> {
        let possible_user = self
            .user_repo
            .borrow()
            .find_by_email(data.email.clone());
        if possible_user.is_some() {
            return Err("User already exist");
        }
        self.user_repo
            .borrow_mut()
            .save(User::new(data.email, data.username));
        Ok(true)
    }
}

fn user_repo_factory() -> ConcreteUserRepositoryInstance {
    Rc::new(RefCell::new(ConcreteUserRepository::new()))
}

fn main() {
    let user_repo_ref = user_repo_factory();
    user_repo_ref.borrow_mut().save(User::new(
        String::from("test1@mail.com"),
        String::from("test1"),
    ));
    user_repo_ref.borrow_mut().save(User::new(
        String::from("test2@mail.com"),
        String::from("test2"),
    ));
    user_repo_ref.borrow_mut().save(User::new(
        String::from("test3@mail.com"),
        String::from("test3"),
    ));
    let user_found = user_repo_ref
        .borrow_mut()
        .find_by_email(String::from("test1@mail.com"));
    user_found.expect("user not found").print();
    user_repo_ref
        .borrow()
        .get_all()
        .into_iter()
        .for_each(|user| user.print());
    println!("After");
    let mut register_service = RegisterUserApplicatioService::new(user_repo_ref.clone());
    register_service
        .execute(UserData {
            email: String::from("test4@mail.com"),
            username: String::from("test4"),
        })
        .unwrap();
    user_repo_ref
        .borrow()
        .get_all()
        .into_iter()
        .for_each(|user| user.print());
}
