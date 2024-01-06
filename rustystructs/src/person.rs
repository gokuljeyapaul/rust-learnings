use crate::gender::Gender;

#[derive(Debug)]
pub struct Person{
  pub name: String,
  pub age: i64,
  pub gender: Gender
}