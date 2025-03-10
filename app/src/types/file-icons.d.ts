declare module "@exuanbo/file-icons-js" {
    type GetClassOptions<Array extends boolean = false> = {
      color?: boolean;
      array?: Array;
    }

    const FileIcons: {
      getClass(name: string, options?: GetClassOptions<false>): Promise<string>;
      getClass(name: string, options?: GetClassOptions<true>): Promise<string[]>;
      getClass(name: string, options?: GetClassOptions<boolean>): Promise<string|string[]>;
    }
    
    export default FileIcons;
}