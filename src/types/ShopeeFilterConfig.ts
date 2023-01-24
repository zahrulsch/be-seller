export interface ShopeeFilterConfig {
  bff_meta: any
  error: any
  error_msg: any
  data: Data
}

export interface Data {
  filter_configuration: FilterConfiguration
  sub_errs: any
  bff_ab_test_sign: string
}

export interface FilterConfiguration {
  filters: Filter[]
  filter_groups: FilterGroup[]
  filter_shortcut_groups: FilterShortcutGroup[]
  dynamic_filter_group_data: DynamicFilterGroupData
}

export interface Filter {
  name: string
  type: number
  label_id?: number
  is_dynamic: boolean
  translations: Translation[]
  icon_url: any
  curation_id: any
  global_attr_id: any
  item_tag_id: any
}

export interface Translation {
  language_code: string
  text: string
  short_text?: string
  is_default: boolean
}

export interface FilterGroup {
  name: string
  filters: Filter2[]
  logic: number
  hidden: boolean
  translations: Translation2[]
  collapse_settings?: CollapseSettings
}

export interface Filter2 {
  name: string
  hidden?: boolean
  schedule_settings: any
}

export interface Translation2 {
  language_code: string
  text: string
  short_text: any
  is_default: boolean
}

export interface CollapseSettings {
  start_offset: number
  display_limit: number
}

export interface FilterShortcutGroup {
  is_default: boolean
  filters: Filter3[]
  schedule_settings: any
}

export interface Filter3 {
  name: string
  hidden: boolean
}

export interface DynamicFilterGroupData {
  facets: Facet[]
  locations: Location[]
  brands: Brand[]
  shippings: Shipping[]
  price_ranges: any
  filter_algorithm: string
  attributes: any
  cc_installment_info: CcInstallmentInfo
}

export interface Facet {
  category: Category
  catid: number
  count: number
  show_parent_category?: boolean
}

export interface Category {
  parentids: number[]
  display_name: string
  is_default_subcat: any
  parent_category_detail?: ParentCategoryDetail
  name: string
}

export interface ParentCategoryDetail {
  category: Category2
  catid: number
  count: number
  show_parent_category: any
}

export interface Category2 {
  parentids: any
  display_name: string
  is_default_subcat: any
  parent_category_detail: any
  name: string
}

export interface Location {
  name: string
  level: string
  sub_location: any
  display_name: string
  translations: any
  tag_ids: number[]
}

export interface Brand {
  brandid: number
  name: string
}

export interface Shipping {
  positionid: number
  name: string
  channelids: string
  display_name: string
  item_tag_ids: number[]
}

export interface CcInstallmentInfo {
  is_cc_installment_payment_eligible: any
  is_non_cc_installment_payment_eligible: any
}
