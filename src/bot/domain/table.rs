use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct TeamTable {
    pub name: String,
    pub created_at: String,
}


#[derive(Debug, Tabled)]
pub struct MemberTable {
    pub name: String,
    pub created_at: String,
}
