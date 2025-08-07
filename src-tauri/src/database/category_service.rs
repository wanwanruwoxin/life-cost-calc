use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{category, Category};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    pub category_id: String,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub category_type: String,
}

pub async fn get_categories(
    db: &DatabaseConnection,
    category_type: Option<String>,
) -> Result<Vec<category::Model>, DbErr> {
    let mut query = Category::find();

    if let Some(cat_type) = category_type {
        query = query.filter(category::Column::CategoryType.eq(cat_type));
    }

    query.all(db).await
}

pub async fn get_category_by_id(
    db: &DatabaseConnection,
    category_id: &str,
) -> Result<Option<category::Model>, DbErr> {
    Category::find()
        .filter(category::Column::CategoryId.eq(category_id))
        .one(db)
        .await
}

pub async fn create_category(
    db: &DatabaseConnection,
    category_data: CategoryData,
) -> Result<category::Model, DbErr> {
    let now = chrono::Utc::now().into();
    let category = category::ActiveModel {
        category_id: Set(category_data.category_id),
        name: Set(category_data.name),
        icon: Set(category_data.icon),
        color: Set(category_data.color),
        category_type: Set(category_data.category_type),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    category.insert(db).await
}

pub async fn update_category(
    db: &DatabaseConnection,
    category_id: &str,
    category_data: CategoryData,
) -> Result<category::Model, DbErr> {
    let category: Option<category::Model> = Category::find()
        .filter(category::Column::CategoryId.eq(category_id))
        .one(db)
        .await?;

    if let Some(category) = category {
        let mut category: category::ActiveModel = category.into();
        category.name = Set(category_data.name);
        category.icon = Set(category_data.icon);
        category.color = Set(category_data.color);
        category.category_type = Set(category_data.category_type);
        category.updated_at = Set(chrono::Utc::now().into());

        category.update(db).await
    } else {
        Err(DbErr::RecordNotFound("Category not found".into()))
    }
}

pub async fn delete_category(db: &DatabaseConnection, category_id: &str) -> Result<(), DbErr> {
    Category::delete_many()
        .filter(category::Column::CategoryId.eq(category_id))
        .exec(db)
        .await?;

    Ok(())
}

// 初始化默认分类数据
pub async fn initialize_default_categories(db: &DatabaseConnection) -> Result<(), DbErr> {
    // 检查是否已经有数据
    let count = Category::find().count(db).await?;
    if count > 0 {
        return Ok(());
    }

    // 支出分类
    let expense_categories = vec![
        ("food", "餐饮", "restaurant", "orange"),
        ("shopping", "购物", "shopping_cart", "pink"),
        ("daily", "日用", "home", "blue"),
        ("transport", "交通", "directions_car", "green"),
        ("vegetables", "蔬菜", "eco", "green"),
        ("fruits", "水果", "apple", "red"),
        ("snacks", "零食", "cookie", "brown"),
        ("sports", "运动", "fitness_center", "blue"),
        ("entertainment", "娱乐", "movie", "purple"),
        ("communication", "通讯", "phone", "blue"),
        ("clothing", "服饰", "checkroom", "pink"),
        ("beauty", "美容", "face", "pink"),
        ("housing", "住房", "house", "brown"),
        ("household", "居家", "chair", "grey"),
        ("children", "孩子", "child_care", "yellow"),
        ("elderly", "长辈", "elderly", "grey"),
        ("social", "社交", "group", "blue"),
        ("travel", "旅行", "flight", "cyan"),
        ("tobacco", "烟酒", "local_bar", "red"),
        ("digital", "数码", "devices", "blue"),
        ("car", "汽车", "directions_car", "grey"),
        ("medical", "医疗", "local_hospital", "red"),
        ("books", "书籍", "book", "brown"),
        ("study", "学习", "school", "blue"),
        ("pets", "宠物", "pets", "orange"),
        ("gift_money", "礼金", "card_giftcard", "red"),
        ("gifts", "礼物", "redeem", "pink"),
        ("office", "办公", "work", "grey"),
        ("repair", "维修", "build", "orange"),
        ("donation", "捐赠", "volunteer_activism", "green"),
        ("lottery", "彩票", "casino", "yellow"),
        ("friends", "亲友", "family_restroom", "blue"),
        ("express", "快递", "local_shipping", "brown"),
        ("settings", "设置", "settings", "grey"),
    ];

    for (id, name, icon, color) in expense_categories {
        create_category(
            db,
            CategoryData {
                category_id: id.to_string(),
                name: name.to_string(),
                icon: icon.to_string(),
                color: color.to_string(),
                category_type: "expense".to_string(),
            },
        )
        .await?;
    }

    // 收入分类
    let income_categories = vec![
        ("salary", "工资", "work", "green"),
        ("bonus", "奖金", "star", "yellow"),
        ("investment", "投资", "trending_up", "blue"),
        ("part_time", "兼职", "schedule", "orange"),
        ("gift", "礼金", "card_giftcard", "red"),
        ("other", "其他", "more_horiz", "grey"),
    ];

    for (id, name, icon, color) in income_categories {
        create_category(
            db,
            CategoryData {
                category_id: id.to_string(),
                name: name.to_string(),
                icon: icon.to_string(),
                color: color.to_string(),
                category_type: "income".to_string(),
            },
        )
        .await?;
    }

    Ok(())
}
