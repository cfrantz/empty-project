#include "proto/python_support.h"
#include "absl/log/log.h"
#include <map>

namespace internal {
const google::protobuf::python::PyProto_API *api;

void init() {
    static int once;
    if (!once) {
          api = static_cast<const google::protobuf::python::PyProto_API*>(PyCapsule_Import(google::protobuf::python::PyProtoAPICapsuleName(), 0));
          if (!api) { 
                LOG(FATAL) << "Failed to import PyProtoAPI";
          }
          once = 1;
    }
}

void remember_descriptor(google::protobuf::Message* m) {
    static std::map<const google::protobuf::DescriptorPool*, PyObject*> pools;
    const google::protobuf::DescriptorPool* pool = m->GetDescriptor()->file()->pool();
    if (pools.find(pool) == pools.end()) {
        pools.emplace(std::make_pair(pool, api->DescriptorPool_FromPool(pool)));
    }
}

}  // namespace
