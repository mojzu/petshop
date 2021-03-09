import { Injectable, Provider, forwardRef } from "@angular/core";
import {
  HttpEvent,
  HttpInterceptor,
  HttpHandler,
  HttpRequest,
  HTTP_INTERCEPTORS,
} from "@angular/common/http";
import { Observable } from "rxjs";

// FIXME: This is added to the output of ng-swagger, it will add cookie credentials
// to http client requests which isn't a built in option in the generated code

export const API_CREDENTIALS_INTERCEPTOR_PROVIDER: Provider = {
  provide: HTTP_INTERCEPTORS,
  useExisting: forwardRef(() => ApiCredentialsInterceptor),
  multi: true,
};

@Injectable()
export class ApiCredentialsInterceptor implements HttpInterceptor {
  intercept(
    req: HttpRequest<any>,
    next: HttpHandler
  ): Observable<HttpEvent<any>> {
    req = req.clone({
      withCredentials: true,
    });
    return next.handle(req);
  }
}
