import {Empty} from "google-protobuf/google/protobuf/empty_pb";
import {Struct} from "google-protobuf/google/protobuf/struct_pb";
import {Configuration, PetshopApi} from "../typescript-axios";
import {Category, Echo, FindByStatus, FindByTag, Pet, Status, Tag} from "../grpc-web/api_pb";
import {PetshopPromiseClient} from "../grpc-web/api_grpc_web_pb";
import {HttpBody} from "../grpc-web/google/api/httpbody_pb";

window["HttpClientClass"] = PetshopApi;
window["httpClient"] = new PetshopApi(new Configuration({
    baseOptions: {withCredentials: true},
    basePath: "http://localhost:10000"
}));

window["GrpcClientClass"] = PetshopPromiseClient;
window["grpcClient"] = new PetshopPromiseClient("http://localhost:10000", null, {
    withCredentials: true,
});

window["Empty"] = Empty;
window["Struct"] = Struct;
window["Category"] = Category;
window["FindByStatus"] = FindByStatus;
window["FindByTag"] = FindByTag;
window["Pet"] = Pet;
window["Status"] = Status;
window["Tag"] = Tag;
window["HttpBody"] = HttpBody;
window["Echo"] = Echo;
