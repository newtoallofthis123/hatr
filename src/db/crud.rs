// add to db

use super::init::Todo;
use sqlx::Row;

//* Add to DB */
pub async fn add_to_db(doc:&Todo){
    let mut conn = super::init::connect().await;
    let _ = sqlx::query(
        r#"
        INSERT INTO todo (title, completed) VALUES (?, ?)
        "#,
    ).bind(&doc.title).bind(&doc.completed).execute(&mut conn).await.unwrap();
}

//* Get all from DB */
pub async fn get_all()->Vec<Todo>{
    let mut conn = super::init::connect().await;
    let rows = sqlx::query(
        r#"
        SELECT * FROM todo
        "#,
    ).fetch_all(&mut conn).await.unwrap();
    let mut todos = Vec::new();
    for row in rows{
        let todo = Todo{
            id: row.get("id"), // change field name to match column name
            title: row.get("title"),
            completed: row.get("completed"),

        };
        todos.push(todo);
    }
    todos
}

//* Delete from DB */
pub async fn delete_from_db(id: i32)->bool{
    if !check_id(id).await{
        return false;
    }
    let mut conn = super::init::connect().await;
    let _ = sqlx::query(
        r#"
        DELETE FROM todo WHERE id = ?
        "#,
    ).bind(id).execute(&mut conn).await.unwrap();
    return true;
}

//*Check if entry with id exists */
pub async fn check_id(id: i32)->bool{
    let mut conn = super::init::connect().await;
    let rows = sqlx::query(
        r#"
        SELECT * FROM todo WHERE id = ?
        "#,
    ).bind(id).fetch_all(&mut conn).await.unwrap();
    if rows.len() == 0{
        false
    }else{
        true
    }
}

//* Edit from DB */
pub async fn edit_from_db(doc:&Todo)->bool{
    if !check_id(doc.id).await{
        return false;
    }
    let mut conn = super::init::connect().await;
    let _ = sqlx::query(
        r#"
        UPDATE todo SET title = ?, completed = ? WHERE id = ?
        "#,
    ).bind(&doc.title).bind(&doc.completed).bind(&doc.id).execute(&mut conn).await.unwrap();
    return true;
}

//* Set as completed */
pub async fn set_completed(id: i32)->bool{
    if !check_id(id).await{
        return false;
    }
    let mut conn = super::init::connect().await;
    let _ = sqlx::query(
        r#"
        UPDATE todo SET completed = ? WHERE id = ?
        "#,
    ).bind(true).bind(id).execute(&mut conn).await.unwrap();
    return true;
}