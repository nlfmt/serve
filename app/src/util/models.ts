export type FileInfo = {
  name: string;
  size: number;
  modified?: number;
  created?: number;
  is_symlink: boolean;
};
export type FolderInfo = {
  name: string;
  modified?: number;
  created?: number;
  is_symlink: boolean;
};
export type DirInfo = {
  files: FileInfo[]
  dirs: FolderInfo[]
}
export type EntryProperties = {
  accessed?: number;
  readonly: boolean;
}