import os as s
cargo_set="""
[package]
name = "rust-base"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
anyhow = "1"
"""
url="https://github.com/jeremychone-channel/rust-base.git "
print("say a folder name")
foldername=input()
cargo_new=cargo_set.replace("rust-base",foldername)
s.system("git clone "+url+foldername)
cargo="./"+foldername+"/"+"Cargo.toml"
s.system("del "+cargo)
s.system("echo "">"+cargo)
file=open(cargo,"w")
file.write(cargo_new)
file.close()









