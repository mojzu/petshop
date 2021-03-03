import "jasmine";
import { env } from "process";
import { Struct } from "google-protobuf/google/protobuf/struct_pb";
import { Configuration, PetshopApi } from "../axios";
import { PetshopPromiseClient } from "../grpc-web/api_grpc_web_pb";

const ENDPOINT = env["CONFIG_ENDPOINT"] || "http://localhost:10000";

const HTTP_CLIENT = new PetshopApi(new Configuration({ basePath: ENDPOINT }));

// FIXME: XMLHttpRequest polyfill for grpc-web client support in node
global.XMLHttpRequest = require('xhr2');
const GRPC_CLIENT = new PetshopPromiseClient(ENDPOINT);

describe("example test", () => {
    beforeAll(() => {
        console.log(`Endpoint: ${ENDPOINT}`);
    });

    it("http client should work", async () => {
        const res = await HTTP_CLIENT.petshopJsonEx({ value: 42 });
        expect(res.data).toEqual({ value: 42 });
    });

    it("user validation to fail", async () => {
        try {
            await HTTP_CLIENT.petshopValidationEx({ email: "notavalidemail", name: "validname" });
            fail();
        } catch (e) {
            expect(e.response.status).toEqual(400);
        }
    });

    it("grpc client should work", async () => {
        const req = Struct.fromJavaScript({ value: 54 });
        const res = await GRPC_CLIENT.jsonEx(req);
        expect(res.toJavaScript()).toEqual({ value: 54 });
    });
});
