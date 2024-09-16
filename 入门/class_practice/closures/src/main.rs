
use std::collections::{HashMap, hash_map};
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
            //当键存在时，输出页面已经渲染
            hash_map::Entry::Occupied(entry) => {
                println!("Page already cached for user: {:?}, article: {:?}", user_id, article_id);
                entry.get().clone() //获取值并克隆
            }
            //不存在时，对页面进行渲染
            hash_map::Entry::Vacant(entry) => {
                println!("Rendering new page for user: {:?}, article: {:?}", user_id, article_id);
                let page_content = (self.render_page)(user_id, article_id);
                entry.insert(page_content.clone()); //插入页面内容
                page_content
            }
        }
    }
}

fn main() {
    // 创建 PageCache 实例，传入渲染函数
    let mut page_cache = PageCache::new(|user_id, article_id| -> String {
        println!("Rendering  for user: {:?}, article: {:?}", user_id, article_id);
        format!("Page for user: {:?}, article: {:?}", user_id, article_id)
    });

// println! 是一个宏，用于向标准输出打印信息。它不生成值，只用于输出调试或信息。不返回值
// format! 是另一个宏，用于生成一个格式化的字符串。它返回一个 String，这个字符串可以被存储、传递或进一步处理。

    // 获取页面内容
    println!("{}", page_cache.get_page("user1".to_string(), 42));
    println!("{}", page_cache.get_page("user1".to_string(), 42));
    println!("{}", page_cache.get_page("user2".to_string(), 42));
}




