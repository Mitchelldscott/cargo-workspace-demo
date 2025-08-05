let hal = variable::get("hal_crate");

print(system::command("pwd", []));
print(system::command("ls", []));

let result = system::command("cargo", ["add", "--version"]);
print(`cargo add version: ${result}`);

print("Adding HAL crate: " + hal);
let result = system::command("cargo", ["add", hal]);

print(result);
