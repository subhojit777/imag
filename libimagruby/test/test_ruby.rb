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

def has_instance_method klass, meth
  works "#{klass}.instance_methods.include? #{meth}",
    (klass.instance_methods.include? meth)
end

puts "---"

[
    :RImag,
    :RStoreId,
    :RStoreHandle,
    :RFileLockEntryHandle,
    :REntryHeader,
    :REntryContent,
    :RImagError,
    :RImagObjDoesNotExistError,
    :RImagStoreError,
    :RImagStoreWriteError,
    :RImagStoreReadError,
    :RImagEntryError,
    :RImagEntryHeaderError,
    :RImagEntryHeaderReadError,
    :RImagEntryHeaderWriteError,
    :RImagTypeError,
].each do |sym|
  if Kernel.const_defined? sym
    RImag.info "Exists: #{sym}"
  else
    RImag.error "#{sym} not defined"
  end
end

works "RStoreId.new_baseless", (not RStoreId.new_baseless("test").nil?)

works "RStoreHandle.respond_to? :new", (RStoreHandle.respond_to? :new)

has_instance_method RStoreHandle, :create
has_instance_method RStoreHandle, :get
has_instance_method RStoreHandle, :retrieve
has_instance_method RStoreHandle, :delete
has_instance_method RStoreHandle, :update
has_instance_method RStoreHandle, :move_by_id
has_instance_method RStoreHandle, :save_as
has_instance_method RStoreHandle, :save_to

has_instance_method RFileLockEntryHandle, :content
has_instance_method RFileLockEntryHandle, :content=
has_instance_method RFileLockEntryHandle, :header
has_instance_method RFileLockEntryHandle, :header=

has_instance_method REntryHeader, :read
has_instance_method REntryHeader, :[]
has_instance_method REntryHeader, :set
has_instance_method REntryHeader, :[]=
has_instance_method REntryHeader, :insert

works "REntryContent.superclass == String", (REntryContent.superclass == String)

