export interface InnerData {
  name: string;
  age: number;
  free: boolean;
  likes: string[];
  skills: { name: string; detail: string }[];
}

export interface UserData {
  data: InnerData;
}
