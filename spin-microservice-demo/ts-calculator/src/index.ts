import { HandleRequest, HttpRequest, HttpResponse, Router } from "@fermyon/spin-sdk"

let router = Router()

function handleDefaultRoute() {
  return {
    status: 200,
    headers: { "content-type": "text/plain" },
    body: "Hello from Default Route"
  }
}

function handleHomeRoute(id: string) {
  return {
    status: 200,
    headers: { "content-type": "text/plain" },
    body: "Hello from Home Route with id:" + id
  }
}

router.get("/ts-calculator/", handleDefaultRoute)
router.get("/ts-calculator/home/:id", ({ params }) => handleHomeRoute(params.id))

export const handleRequest: HandleRequest = async function(request: HttpRequest): Promise<HttpResponse> {
  return await router.handleRequest(request)
}
