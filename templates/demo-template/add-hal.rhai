let hal = variable::get("hal_crate");

let files = file::listdir();
for f in files {
    print(`file: ${f}`);
}

print("Adding HAL crate: " + hal);
let result = system::command("cargo", ["add", hal]);

print(result);
