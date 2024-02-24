-- build Rust plugin
local build_command = "cargo build --release > /dev/null 2>&1"
local result = os.execute(build_command)

if result == 0 then
  print("Build succeeded")
else
  print("Build failed")
end
