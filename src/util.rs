//! polkit model — privilege authorization framework.

#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub description: String,
    pub defaults: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Polkit {
    pub actions: Vec<Action>,
}

impl Default for Polkit {
    fn default() -> Self { Self { actions: vec![
        Action { id: "org.freedesktop.systemd1.manage-units".into(), description: "Manage systemd units".into(), defaults: vec!["auth_admin".into()] },
        Action { id: "org.freedesktop.login1.reboot".into(), description: "Reboot system".into(), defaults: vec!["auth_admin".into()] },
    ] } }
}

impl Polkit {
    pub fn new() -> Self { Self::default() }
    pub fn list_actions(&self) -> &[Action] { &self.actions }
    pub fn authorize(&self, action_id: &str, user: &str) -> bool {
        self.actions.iter().any(|a| a.id == action_id) && !user.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actions_auth() {
        let p = Polkit::new();
        assert_eq!(p.list_actions().len(), 2);
        assert!(p.authorize("org.freedesktop.systemd1.manage-units", "root"));
        assert!(!p.authorize("bad.action", "user"));
    }
}
