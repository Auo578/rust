// use std::collections::btree_map::Entry;
// use std::collections::HashMap;
// use std::hash::Hash;

// fn main() {
//     // let mut cache = PageCache::new();

//     // let user_id = 1;
//     // let article_id = 101;

//     // // 第一次获取页面，渲染新页面并插入缓存
//     // let page1 = cache.get_page(user_id, article_id, || {
//     //     "Rendered page for user 1 and article 101".to_string()
//     // });
//     // println!("{}", page1);

//     // // 再次获取同一页面，应该命中缓存
//     // let page2 = cache.get_page(user_id, article_id, || {
//     //     "This should not be rendered".to_string()
//     // });
//     // println!("{}", page2);
//     let mut page_cache = PageCache::new(|user_id, article_id| -> String{
//         println!("Rendering page for user: {:?}, article: {:?}",user_id, article_id)
//         format!("Page for user: {:?}, article: {:?}",user_id, article_id)
//     });

//     println!("{}",page_cache.get_page("user1", 42));

//     println!("{}",page_cache.get_page("user1", 42));

//     println!("{}",page_cache.get_page("user2", 42));



// } 

// #[derive(Debug)]
// struct PageCache<U,A>{
//     cache: HashMap<(U,A), String>,
// }

// impl <U,A> PageCache<U,A>
// where 
//     U: Hash + Eq + Clone, 
//     A: Hash + Eq + Clone,
// {
//     fn new() -> Self{
//         Self{
//             cache: HashMap::new(),
//         }
//     }

//     fn get_page(&mut self, user_id: U, article_id: A) -> String {
//         let key = (user_id.clone(), article_id.clone());
//         match self.cache.entry(key) {
//             Entry::Occupied(entry) => {
//                 println!("Already insert for user: {:?}, article: {:?}",user_id, article_id);
//                 entry.get.clone()
//             }
//             Entry::Vacant(entry) => {
//                 println!("Inserting new page for user: {:?}, article: {:?}", user_id, article_id);
//                 let page_content = render_page();
//                 entry.insert(page_content.clone());
//                 page_content
//             }
//         }
//     }
// }











use std::collections::HashMap;
use std::hash::Hash;


// 定义 PageCache 结构体
struct PageCache<U, A, F>
where
    U: Eq + Hash + Clone,
    A: Eq + Hash + Clone,
    F: Fn(U, A) -> String,
{
    cache: HashMap<(U, A), String>,
    render_page: F,
}

impl<U, A, F> PageCache<U, A, F>
where
    U: Eq + Hash + Clone + std::fmt::Debug,
    A: Eq + Hash + Clone + std::fmt::Debug,
    F: Fn(U, A) -> String,
{
    // 创建 PageCache 实例的函数
    fn new(render_page: F) -> Self {
        PageCache {
            cache: HashMap::new(),
            render_page,
        }
    }

    // 获取页面的函数
    fn get_page(&mut self, user_id: U, article_id: A) -> String {
        let key = (user_id.clone(), article_id.clone());

        match self.cache.entry(key) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                println!("Page already cached for user: {:?}, article: {:?}", user_id, article_id);
                entry.get().clone()
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                println!("Rendering new page for user: {:?}, article: {:?}", user_id, article_id);
                let page_content = (self.render_page)(user_id, article_id);
                entry.insert(page_content.clone());
                page_content
            }
        }
    }
}

fn main() {
    // 创建 PageCache 实例，传入渲染函数
    let mut page_cache = PageCache::new(|user_id, article_id| -> String {
        println!("Rendering page for user: {:?}, article: {:?}", user_id, article_id);
        format!("Page for user: {:?}, article: {:?}", user_id, article_id)
    });

    // 获取页面内容
    println!("{}", page_cache.get_page("user1".to_string(), 42));
    println!("{}", page_cache.get_page("user1".to_string(), 42));
    println!("{}", page_cache.get_page("user2".to_string(), 42));
}




