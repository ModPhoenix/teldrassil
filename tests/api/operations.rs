pub const SIGN_UP_MUTATION: &str = r#"
mutation SignUp($email: String!, $username: String!, $password: String!) {
  signUp(email: $email, username: $username, password: $password)
}
"#;

pub const SIGN_IN_MUTATION: &str = r#"
mutation SignIn($email: String!, $password: String!) {
  signIn(email: $email, password: $password)
}
"#;

pub const ME_QUERY: &str = r#"
query Me {
  me {
    id
    email
    username
    createdAt
    updatedAt
  }
}
"#;

pub const NODE_QUERY: &str = r#"
query Node($where: NodeWhere!) {
  node(where: $where) {
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

pub const CREATE_NODE_MUTATION: &str = r#"
mutation CreateNode($parentId: UUID!, $name: String!, $content: String!) {
  createNode(parentId: $parentId, name: $name, content: $content) {
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

pub const UPDATE_NODE_MUTATION: &str = r#"
mutation UpdateNode($id: UUID!, $name: String!, $content: String!) {
  updateNode(id: $id, name: $name, content: $content) {
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

pub const DELETE_NODE_MUTATION: &str = r#"
mutation DeleteNode($id: UUID!) {
  deleteNode(id: $id)
}
"#;
