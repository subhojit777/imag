#!/usr/bin/env ruby

# imag ruby interface module
#
# This module is created because the library which is used to write the Ruby
# bindings in Rust does not support modules.
#
# This is a wrapper to have nice Ruby-like things in the Ruby codebase and for
# beeing backwards compatible as soon as the Rust library gets module support.
#
# There will probably always be a wrapper for the Rust library, to be more
# flexible with the API, though.
module Imag

  # Function name of the function to call to initialize the Rust backend part.
  # Do not use.
  IMAG_INIT_FN_NAME = 'imag_ruby_initialize'

  # Setup method
  #
  # Call this method for initializing the library.
  # It dynamically creates the classes for the imag library.
  def self.setup binary_path
    require binary_path

    self.core_setup
    self.classes_setup
  end

  # Abstraction over the logger frontend of the binary
  #
  # This is just a translation to nice Ruby code
  module Logger

    def self.init cfg
      debug   = !!cfg[:debug]
      verbose = !!cfg[:verbose]
      color   = !!cfg[:color]
      ::RImag.init_logger debug, verbose, color
    end

    # Log text with "trace" level in imag
    def self.trace msg
      ::RImag.trace msg
    end

    # Log text with "debug" level in imag
    def self.dbg msg
      ::RImag.dbg msg
    end

    # Log text with "debug" level in imag (alias for Imag::Logger::dbg)
    def self.debug msg
      ::RImag.debug msg
    end

    # Log text with "info" level in imag
    def self.info msg
      ::RImag.info msg
    end

    # Log text with "warning" level in imag
    def self.warn msg
      ::RImag.warn msg
    end

    # Log text with "error" level in imag
    def self.error msg
      ::RImag.error msg
    end

  end

  private

  # Class names of the Classes in the Ruby scope
  #
  # These classes are created by the Rust backend with an "R" prefix, and are
  # here mapped to Ruby classes by inheriting from them.
  #
  # Addidional functionality and convenience methods can then be set up upon
  # these pure Ruby classes.
  def self.class_names
    [
      :StoreId             ,
      :StoreHandle         ,
      :FileLockEntryHandle ,
      :EntryHeader         ,
      :EntryContent        ,
    ]
  end

  # Do the core setup
  #
  # Maps the Rust classes to Ruby classes by inheriting from them
  def self.core_setup
    self.class_names.map {|n| [n, "R#{n}".to_sym ] }.each do |elem|
      Imag.const_set elem.first, Kernel.const_get(elem.last)
    end
  end

  # Class setup
  #
  # Summarizing method for calling all the class-setup methods.
  def self.classes_setup
    self.class_storeid_setup
  end

  # Class setup for the StoreId class
  #
  # Sets up additional methods for the Imag::StoreId class.
  def self.class_storeid_setup
    Imag::StoreId.class_exec do
      def to_s
        self.to_str
      end
    end
  end

end

if __FILE__ == $0
  puts "Running some tests..."
  puts "I hope you passed the library object as first argument..."
  begin
    Imag.setup ARGV.first
  rescue Exception => e
    puts "Seems not to be the case... or something else went wrong..."
    puts e
    exit 1
  end

  Imag::Logger.init debug: true, verbose: true, color: true
  Imag::Logger.info "The Logger should work now"

  Imag::Logger.info "Lets see whether we have properly setup StoreId"
  Imag::Logger.info Imag::StoreId::new_baseless("baselessId").to_s
  Imag::Logger.info "Seems good."
end

