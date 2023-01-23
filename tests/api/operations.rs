pub const GET_NODE_QUERY: &str = r#"
query GetNode($id: UUID!) {
  getNode(id: $id) {
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
