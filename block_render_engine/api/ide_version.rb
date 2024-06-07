require 'dtr_core'
require 'dtr_to_rust'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    dtr_core: Gem.loaded_specs['dtr_core'].version,
    dtr_to_rust: Gem.loaded_specs['dtr_to_rust'].version,
    block_render_engine: '0.6.3'
  }.to_json
end
