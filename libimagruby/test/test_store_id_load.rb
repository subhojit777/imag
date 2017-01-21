#!/usr/bin/env ruby

require 'fiddle'

lib = Fiddle::dlopen '../target/release/liblibimagruby.so'
Fiddle::Function::new(lib['imag_ruby_initialize'], [], Fiddle::TYPE_VOIDP).call

works = (not RStoreId.new_baseless("test").nil?)

puts "Works: #{works}"

