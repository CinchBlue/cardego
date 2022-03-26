pub struct Shape {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub image_url: Option<String>,
}
