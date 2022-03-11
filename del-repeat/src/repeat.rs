use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error, Write};
use walkdir::WalkDir;

pub fn find_repeat_file(path: String) -> Result<bool, Error> {
    println!("读取路径文件:{}",path);
    let mut file_map = BTreeMap::new();

    let mut has_reapet = false;

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let file_size = entry.metadata()?.len();
        let file_path = entry.path().display().to_string();
        // 若存在同样的大小,push path到map
        if file_map.get(&file_size).is_none() {
            let tem = vec![file_path];
            file_map.insert(file_size, tem);
        } else {
            let tem = file_map.get_mut(&file_size).unwrap();
            tem.push(file_path);
            has_reapet = true;
        }
    }
    // 由小到大排序
    let path = format!("{}/result.json", &path);
    let mut output = File::create(path)?;
    let str = String::from(format!("{:?}", file_map));
    write!(output, "{}", str)?;
    Ok(has_reapet)
}

#[cfg(test)]
mod tests {
    use super::find_repeat_file;
    #[test]
    fn test() {
        let res = find_repeat_file(String::from("./"));
        println!("{:?}", res)
    }
}
