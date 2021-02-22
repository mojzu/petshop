import {Configuration, PetshopApi} from "../typescript-axios";
import {Category, FindByStatus, Pet, Status, Tag} from "../grpc-web/api_pb";
import {PetshopPromiseClient} from "../grpc-web/api_grpc_web_pb";
import {HttpBody} from "../grpc-web/google/api/httpbody_pb";

window["HttpClientClass"] = PetshopApi;
window["httpClient"] = new PetshopApi(new Configuration({basePath: "http://localhost:10000"}));

window["GrpcClientClass"] = PetshopPromiseClient;
window["grpcClient"] = new PetshopPromiseClient("http://localhost:10000");

window["Category"] = Category;
window["FindByStatus"] = FindByStatus;
window["Pet"] = Pet;
window["Status"] = Status;
window["Tag"] = Tag;
window["HttpBody"] = HttpBody;
