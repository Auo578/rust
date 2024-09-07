// //课程作业1
// use std::time::Instant;

// fn main(){
//     let start_heap = Instant::now();
//     let a: Box<[i32]> = vec![2;1_000_000].into_boxed_slice();
//     let duration_heap = start_heap.elapsed();
    
//     let start_access_heap = Instant::now();
//     let sum_heap: i32 = a.iter().sum();
//     let duration_access_heap = start_access_heap.elapsed();

//     println!("Heap allocation time: {:?}",duration_heap);
//     println!("Heap access time: {:?}",duration_access_heap);
//     println!("The Heap sum is {:?}",sum_heap);

//     // let start_stack = Instant::now();
//     // let b=[2;1_000_000];
//     // let duration_stack = start_stack.elapsed();

//     // let start_access_stack = Instant::now();
//     // let sum_stack: i32 = b.iter().sum();
//     // let duration_access_stack = start_access_stack.elapsed();

//     // println!("Heap allocation time: {:?}",duration_stack);
//     // println!("Heap access time: {:?}",duration_access_stack);
//     // println!("The Heap is {:?}",b);

// }

//课程作业2

pub trait FileSystem {
    fn create_file(&mut self, name:&str);
    fn create_folder(&mut self, name:&str);
    fn list_contents(&self);
}
#[derive(Clone)]
pub enum Node{
    File(String),
    Folder(String,Vec<Box<Node>>),
}
    
pub struct FolderNode{
    name:String,
    contents:Vec<Box<Node>>,
}

impl FolderNode {
    pub fn new(name:&str) -> Self{
        FolderNode{
            name:name.to_string(),
            contents:Vec::new(),
        }
    }

    fn find_folder(&mut self, name: &str) -> Option<Box<FolderNode> > {
        for node in self.contents.iter_mut() {
            if let Node::Folder(ref mut folder_node,_) = **node {
                if folder_node == name {
                    return Some(Box::new(FolderNode{
                        name:folder_node.clone(),
                        contents:Vec::new(),
                    }));
                }
            }
        }
        None
    }

    // fn find_folder(&mut self, name: &str) -> Option<&mut FolderNode> {
    //     // 首先检查当前文件夹
    //     if self.name == name {
    //         return Some(self);
    //     }

    //     // 遍历当前文件夹中的内容
    //     for node in self.contents.iter_mut() {
    //         if let Node::Folder(_, ref mut sub_contents) = &mut **node {
    //             // 递归查找子文件夹
    //             if let Some(found) = Self::find_folder_in_contents(sub_contents, name) {
    //                 return Some(found);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn find_folder_in_contents<'a>(
    //     contents: &'a mut Vec<Box<Node>>,
    //     name: &str
    // ) -> Option<&'a mut FolderNode> {
    //     for node in contents.iter_mut() {
    //         if let Node::Folder(folder_name, ref mut sub_contents) = &mut **node {
    //             if folder_name == name {
    //                 // 返回对当前文件夹的可变引用
    //                 return Some(folder_name, sub_contents);
    //             } else if let Some(found) = Self::find_folder_in_contents(sub_contents, name) {
    //                 return Some(found);
    //             }
    //         }
    //     }
    //     None
    // }
}




impl FileSystem for FolderNode {
    fn create_file(&mut self, name:&str) {
        self.contents.push(Box::new(Node::File(name.to_string())));
    }

    fn create_folder(&mut self, name:&str) {
        self.contents.push(Box::new(Node::Folder(name.to_string(),Vec::new())));
    }
    
    fn list_contents(&self) {
        for node in &self.contents {
            match node.as_ref() {    //注意
                Node::File(name) => println!("File: {}",name),
                Node::Folder(name,_) => println!("Folder: {}",name),
            }
        }
    }
}

fn main() {
    let mut root = FolderNode::new("root");

    root.create_file("file1.txt");
    root.create_folder("subfolder1");

    if let Some(mut subfolder) = root.find_folder("subfolder1") {
        subfolder.create_file("file2.txt");
    }

    println!("Contents of root folder:");
    root.list_contents();

    if let Some(mut subfolder) = root.find_folder("subfolder1") {
        println!("\nContents of subfolder1:");
        subfolder.list_contents();
    }
}





