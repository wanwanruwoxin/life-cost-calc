// 数据库相关类型定义

export interface Category {
  id?: number;
  category_id: string;
  name: string;
  icon: string;
  color: string;
  category_type: "expense" | "income";
  created_at?: string;
  updated_at?: string;
}

export interface CategoryData {
  category_id: string;
  name: string;
  icon: string;
  color: string;
  category_type: "expense" | "income";
}

export interface ExpenseRecord {
  id?: number;
  record_type: "expense" | "income";
  category_id: string;
  amount: string; // Decimal 作为字符串传输
  note?: string | null;
  created_at?: string;
  updated_at?: string;
}

export interface ExpenseRecordData {
  record_type: "expense" | "income";
  category_id: string;
  amount: string;
  note?: string | null;
}

export interface RecordFilter {
  record_type?: "expense" | "income";
  category_id?: string;
  start_date?: string; // ISO 8601 格式
  end_date?: string; // ISO 8601 格式;
}

// Tauri Commands 类型定义
export interface DatabaseCommands {
  // Category commands
  get_categories: (args: {
    categoryType?: "expense" | "income";
  }) => Promise<Category[]>;
  get_category_by_id: (args: {
    categoryId: string;
  }) => Promise<Category | null>;
  create_category: (args: { categoryData: CategoryData }) => Promise<Category>;
  update_category: (args: {
    categoryId: string;
    categoryData: CategoryData;
  }) => Promise<Category>;
  delete_category: (args: { categoryId: string }) => Promise<void>;

  // Record commands
  get_records: (args: { filter?: RecordFilter }) => Promise<ExpenseRecord[]>;
  get_record_by_id: (args: { id: number }) => Promise<ExpenseRecord | null>;
  create_record: (args: {
    recordData: ExpenseRecordData;
  }) => Promise<ExpenseRecord>;
  update_record: (args: {
    id: number;
    recordData: ExpenseRecordData;
  }) => Promise<ExpenseRecord>;
  delete_record: (args: { id: number }) => Promise<void>;
  get_statistics: (args: {
    recordType?: "expense" | "income";
    startDate?: string;
    endDate?: string;
  }) => Promise<string>;
}
