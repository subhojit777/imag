#!/usr/bin/env ruby

require "../target/debug/liblibimagruby.so"

color = true
verbose = true
debug = true

RImag.init_logger debug, verbose, color

RImag.trace  "Trace-Hello from Ruby"
RImag.dbg    "Debug-Hello from Ruby"
RImag.debug  "Debug-Hello from Ruby"
RImag.info   "Info-Hello from Ruby"
RImag.warn   "Warn-Hello from Ruby"
RImag.error  "Error-Hello from Ruby"

def works name, b
  if b
    RImag.info "Works: #{name}"
  else
    RImag.error "Fails: #{name}"
  end
end

puts "---"

works "RStoreId.new_baseless"                              , (not RStoreId.new_baseless("test").nil?)
works "RStoreHandle.respond_to? :new"                      , (RStoreHandle.respond_to? :new)
works "RStoreHandle.instance_methods.include? :create"     , (RStoreHandle.instance_methods.include? :create)
works "RStoreHandle.instance_methods.include? :get"        , (RStoreHandle.instance_methods.include? :get)
works "RStoreHandle.instance_methods.include? :retrieve"   , (RStoreHandle.instance_methods.include? :retrieve)
works "RStoreHandle.instance_methods.include? :delete"     , (RStoreHandle.instance_methods.include? :delete)
works "RStoreHandle.instance_methods.include? :update"     , (RStoreHandle.instance_methods.include? :update)
works "RStoreHandle.instance_methods.include? :move_by_id" , (RStoreHandle.instance_methods.include? :move_by_id)
works "RStoreHandle.instance_methods.include? :save_as"    , (RStoreHandle.instance_methods.include? :save_as)
works "RStoreHandle.instance_methods.include? :save_to"    , (RStoreHandle.instance_methods.include? :save_to)

