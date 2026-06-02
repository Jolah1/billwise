// Wire types — mirror backend/src/handlers/{auth,projects,sections,items}.rs.
// Money is integer kobo end-to-end; quantity is a decimal string to
// avoid JS float precision loss. Both invariants are enforced server-
// side too, but the type signatures here keep them visible at call
// sites.

export type ItemType =
  | 'measured'
  | 'provisional_sum'
  | 'pc_sum'
  | 'prime_cost';

export interface AuthResp {
  account_id: string;
  token: string;
}

export interface RegisterReq {
  email: string;
  password: string;
}

export interface LoginReq {
  email: string;
  password: string;
}

export interface ProjectResp {
  id: string;
  title: string;
  client: string;
  location: string;
  project_date: string; // ISO 8601 date, e.g. "2026-06-02"
  created_at: string;   // RFC3339, e.g. "2026-06-02T21:51:05.576882Z"
}

export interface ProjectDetailResp extends ProjectResp {
  total_kobo: number;
  sections: SectionWithItemsResp[];
}

export interface SectionWithItemsResp {
  id: string;
  name: string;
  sort_order: number;
  subtotal_kobo: number;
  items: ItemInDetailResp[];
}

export interface ItemInDetailResp {
  id: string;
  description: string;
  quantity: string;
  unit: string;
  rate_kobo: number;
  item_type: ItemType;
  sort_order: number;
  amount_kobo: number;
}

export interface CreateProjectReq {
  title: string;
  client: string;
  location: string;
  project_date: string;
}

export interface SectionResp {
  id: string;
  project_id: string;
  name: string;
  sort_order: number;
}

export interface CreateSectionReq {
  name: string;
}

export interface ItemResp {
  id: string;
  section_id: string;
  description: string;
  quantity: string;
  unit: string;
  rate_kobo: number;
  item_type: ItemType;
  sort_order: number;
  amount_kobo: number;
}

export interface CreateItemReq {
  description: string;
  quantity: string;
  unit: string;
  rate_kobo: number;
  item_type: ItemType;
}

// Backend AppError envelope.
export interface ApiErrorBody {
  error: string;
  message: string;
}
