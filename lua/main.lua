local exports = {}

local function extend_exports(modname)
	for k, v in pairs(require(modname)) do
		exports[k] = v
	end
end

exports.foo = function()
	return "Foo!"
end

extend_exports("lua/example")

return exports
