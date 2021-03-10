import { Empty } from "google-protobuf/google/protobuf/empty_pb";
import { Struct } from "google-protobuf/google/protobuf/struct_pb";
import { Configuration, PetshopApi } from "../clients/axios";
import {
    Category,
    Echo,
    User,
    FindByStatus,
    FindByTag,
    Pet,
    Status,
    Tag,
} from "../clients/grpc-web/api_pb";
import { PetshopPromiseClient } from "../clients/grpc-web/api_grpc_web_pb";
import { HttpBody } from "../clients/grpc-web/google/api/httpbody_pb";

window["HttpClientClass"] = PetshopApi;
window["httpClient"] = new PetshopApi(
    new Configuration({
        baseOptions: { withCredentials: true },
        basePath: "http://localhost:10000",
    })
);
window["apiHttpClient"] = new PetshopApi(
    new Configuration({
        // FIXME: Use axios baseOptions here instead of apiKey/username/password
        // properties on config, typescript-axios does not use them
        baseOptions: {
            headers: {
                Authorization: "an-example-api-key",
            },
        },
        basePath: "http://localhost:10001",
    })
);

// FIXME: Add unary interceptor to add CSRF token request header, this mimics the behaviour
// of the axios and angular clients. It probably isn't necessary for a grpc-web client but
// it makes CSRF handling on the server more consistent when exposing grpc and transcoded
// json interfaces
//
// In production it would probably make more sense to choose either a grpc-web or http
// client for the user interface to use, and to enable csrf protection if required
const getCookieValue = (name) =>
    document.cookie.match("(^|;)\\s*" + name + "\\s*=\\s*([^;]+)")?.pop() || "";
const CsrfInterceptor = function () {};
CsrfInterceptor.prototype.intercept = function (request, invoker) {
    const xsrfToken = getCookieValue("XSRF-TOKEN");
    if (xsrfToken != null && xsrfToken !== "") {
        const metadata = request.getMetadata();
        metadata["X-XSRF-TOKEN"] = xsrfToken;
    }
    return invoker(request);
};

window["GrpcClientClass"] = PetshopPromiseClient;
window["grpcClient"] = new PetshopPromiseClient(
    "http://localhost:10000",
    null,
    {
        withCredentials: true,
        unaryInterceptors: [new CsrfInterceptor()],
    }
);

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
window["User"] = User;
