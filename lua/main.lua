local exports = {}

exports.foo = function()
	return "Foo!"
end

exports.bar = require("lua/example").bar

return exports
