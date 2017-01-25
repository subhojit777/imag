#!/usr/bin/env ruby

require "../target/debug/liblibimagruby.so"

color   = true
verbose = true
debug   = false

RImag.init_logger debug, verbose, color

store_handle = RStoreHandle::new(false, "/tmp/store")
id = RStoreId::new_baseless("test")
test_handle = store_handle.retrieve(id)
puts "Header: #{test_handle.header.to_s}"
puts "Content: '#{test_handle.content}'"

test_handle.content = "Foo"
test_handle.header = {
  "imag" => {
    "links" => [],
    "version" => "0.2.0"
  },
  "example" => {
    "test" => "foo"
  }
}

