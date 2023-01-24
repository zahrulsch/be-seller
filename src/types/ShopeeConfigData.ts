export interface ShopeeConfigData {
  id?: number;
  created_at?: string;
  name: string;
  seller_types: Array<number|string>;
  price_ranges: Array<number|string>;
  minimum_star: number;
  minimum_sold: number;
  minimum_stock: number;
  sort_by: string;
  shippings: Array<number>;
  exclude_cod: boolean;
  ban_keywords: Array<string>;
  cities: Array<string>;
}
