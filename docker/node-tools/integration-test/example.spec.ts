import "jasmine";
import { env } from "process";
import { Struct } from "google-protobuf/google/protobuf/struct_pb";
import { Configuration, ExampleApi } from "../clients/axios";
import { ExamplePromiseClient } from "../clients/grpc-web/api_grpc_web_pb";

const ENDPOINT = env["CONFIG_ENDPOINT"] || "http://localhost:10000";

const HTTP_CLIENT = new ExampleApi(new Configuration({ basePath: ENDPOINT }));

// FIXME: XMLHttpRequest polyfill for grpc-web client support in node
global.XMLHttpRequest = require("xhr2");
const GRPC_CLIENT = new ExamplePromiseClient(ENDPOINT);

describe("example test", () => {
    beforeAll(() => {
        console.log(`Endpoint: ${ENDPOINT}`);
    });

    it("http client should work", async () => {
        const res = await HTTP_CLIENT.exampleJson({ value: 42 });
        expect(res.data).toEqual({ value: 42 });
    });

    it("user validation to fail", async () => {
        try {
            await HTTP_CLIENT.exampleValidation({
                email: "notavalidemail",
                name: "validname",
            });
            fail();
        } catch (e) {
            expect(e.response.status).toEqual(400);
        }
    });

    it("grpc client should work", async () => {
        const req = Struct.fromJavaScript({ value: 54 });
        const res = await GRPC_CLIENT.json(req);
        expect(res.toJavaScript()).toEqual({ value: 54 });
    });
});
