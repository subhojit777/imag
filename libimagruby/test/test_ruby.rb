#!/usr/bin/env ruby

require 'fiddle'

INIT_FN = 'imag_ruby_initialize'

for targ in %w(debug release)
  begin
    lib = Fiddle::dlopen "../target/#{targ}/liblibimagruby.so"
    Fiddle::Function::new(lib[INIT_FN], [], Fiddle::TYPE_VOIDP).call
    break;
  rescue Fiddle::DLError
  end
end

color = true
verbose = true
debug = true

Imag.init_logger debug, verbose, color

Imag.trace  "Trace-Hello from Ruby"
Imag.dbg    "Debug-Hello from Ruby"
Imag.debug  "Debug-Hello from Ruby"
Imag.info   "Info-Hello from Ruby"
Imag.warn   "Warn-Hello from Ruby"
Imag.error  "Error-Hello from Ruby"

def works name, b
  if b
    Imag.info "Works: #{name}"
  else
    Imag.error "Fails: #{name}"
  end
end

puts "---"

works "RStoreId.new_baseless"                        , (not RStoreId.new_baseless("test").nil?)
works "RStore.instance_methods.include? :create"     , (RStore.instance_methods.include? :create)
works "RStore.instance_methods.include? :get"        , (RStore.instance_methods.include? :get)
works "RStore.instance_methods.include? :retrieve"   , (RStore.instance_methods.include? :retrieve)
works "RStore.instance_methods.include? :delete"     , (RStore.instance_methods.include? :delete)
works "RStore.instance_methods.include? :update"     , (RStore.instance_methods.include? :update)
works "RStore.instance_methods.include? :move_by_id" , (RStore.instance_methods.include? :move_by_id)
works "RStore.instance_methods.include? :save_as"    , (RStore.instance_methods.include? :save_as)
works "RStore.instance_methods.include? :save_to"    , (RStore.instance_methods.include? :save_to)

