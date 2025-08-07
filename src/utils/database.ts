import { invoke } from "@tauri-apps/api/core";
import type {
  Category,
  CategoryData,
  ExpenseRecord,
  ExpenseRecordData,
  RecordFilter,
} from "@/types/database";

// 分类相关操作
export class CategoryService {
  static async getCategories(
    categoryType?: "expense" | "income"
  ): Promise<Category[]> {
    return await invoke("get_categories", { categoryType });
  }

  static async getCategoryById(categoryId: string): Promise<Category | null> {
    return await invoke("get_category_by_id", { categoryId });
  }

  static async createCategory(categoryData: CategoryData): Promise<Category> {
    return await invoke("create_category", { categoryData });
  }

  static async updateCategory(
    categoryId: string,
    categoryData: CategoryData
  ): Promise<Category> {
    return await invoke("update_category", { categoryId, categoryData });
  }

  static async deleteCategory(categoryId: string): Promise<void> {
    return await invoke("delete_category", { categoryId });
  }
}

// 记录相关操作
export class RecordService {
  static async getRecords(filter?: RecordFilter): Promise<ExpenseRecord[]> {
    return await invoke("get_records", { filter });
  }

  static async getRecordById(id: number): Promise<ExpenseRecord | null> {
    return await invoke("get_record_by_id", { id });
  }

  static async createRecord(
    recordData: ExpenseRecordData
  ): Promise<ExpenseRecord> {
    return await invoke("create_record", { recordData });
  }

  static async updateRecord(
    id: number,
    recordData: ExpenseRecordData
  ): Promise<ExpenseRecord> {
    return await invoke("update_record", { id, recordData });
  }

  static async deleteRecord(id: number): Promise<void> {
    return await invoke("delete_record", { id });
  }

  static async getStatistics(
    recordType?: "expense" | "income",
    startDate?: string,
    endDate?: string
  ): Promise<number> {
    const result = await invoke("get_statistics", {
      recordType,
      startDate,
      endDate,
    });
    return parseFloat(result as string);
  }
}

// 便捷方法
export const db = {
  categories: CategoryService,
  records: RecordService,
};
