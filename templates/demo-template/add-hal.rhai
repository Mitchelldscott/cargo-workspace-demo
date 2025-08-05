let hal = variable::get("hal_crate");

let pwd = system::command("echo", ["$PWD"]);
print(`[post-hook] pwd: ${pwd}`);

print("Adding HAL crate: " + hal);
print(system::command("cargo", ["add", hal]));
