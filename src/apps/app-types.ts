export interface AppDefinition {
  id: string;
  name: string;
  executables: string[];
  client_id: string;
  category: string;
  icon?: string;
  icon_url?: string;
}
