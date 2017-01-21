#!/usr/bin/env ruby

require 'fiddle'

lib = Fiddle::dlopen '../target/debug/liblibimagruby.so'
Fiddle::Function::new(lib['imag_ruby_initialize'], [], Fiddle::TYPE_VOIDP).call

works = (not RStoreId.new_baseless("test").nil?)
Imag.init_logger true, true, true
Imag.info "Hello from Ruby"

puts "Works: #{works}"

