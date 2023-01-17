pub const GET_BRANCH_QUERY: &str = r#"
query GetBranch($id: UUID!) {
  getBranch(id: $id) {
    id
    name
    content
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

pub const CREATE_BRANCH_MUTATION: &str = r#"
mutation CreateBranch($parentId: UUID!, $name: String!, $content: String!) {
  createBranch(parentId: $parentId, name: $name, content: $content) {
    id
    name
    content
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
