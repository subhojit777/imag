#!/usr/bin/env ruby

require 'fiddle'

INIT_FN = 'imag_ruby_initialize'

for targ in %w(debug)
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

store_handle = RStoreHandle::new(false, "/tmp/store")
id = RStoreId::new_baseless("test")
test_handle = store_handle.create(id)

Imag.info "Created #{test_handle.location.to_str} from Ruby"

