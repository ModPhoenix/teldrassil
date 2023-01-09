pub const GET_BRANCH: &str = r#"
query GetBranch($id: UUID!) {
  getBranch(id: $id) {
    id
    name
    parents {
      id
      name
    }
    children {
      id
      name
    }
  }
}
    "#;
