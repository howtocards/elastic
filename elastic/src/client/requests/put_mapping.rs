use std::marker::PhantomData;
use serde_json;
use serde::Serialize;

use error::*;
use client::Client;
use client::requests::{Index, Type, IndicesPutMappingRequest, RequestBuilder, RawRequestBuilder};
use client::responses::CommandResponse;
use types::document::{FieldType, DocumentType, IndexDocumentMapping};

/** 
A builder for a [`Client.put_mapping`][Client.put_mapping] request. 

[Client.put_mapping]: ../struct.Client.html#method.put_mapping
*/
pub struct PutMappingRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    _marker: PhantomData<TDocument>,
}

impl Client {
    /** 
    Create a [`RequestBuilder` for a put mapping request][RequestBuilder.put_mapping]. 
    
    # Examples

    Put the document mapping for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = ClientBuilder::new().build().unwrap();
    client.put_mapping::<MyType>(index("myindex"))
          .send()
          .unwrap();
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [RequestBuilder.put_mapping]: requests/struct.RequestBuilder.html#put-mapping-builder
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn put_mapping<'a, TDocument>(&'a self,
                                      index: Index<'static>)
                                      -> RequestBuilder<'a, PutMappingRequestBuilder<TDocument>>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(&self,
                            None,
                            PutMappingRequestBuilder {
                                index: index,
                                ty: ty,
                                _marker: PhantomData,
                            })
    }
}

impl<TDocument> PutMappingRequestBuilder<TDocument>
    where TDocument: DocumentType
{
    fn into_request(self) -> Result<IndicesPutMappingRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&IndexDocumentMapping::from(TDocument::mapping()))?;

        Ok(IndicesPutMappingRequest::for_index_ty(self.index, self.ty, body))
    }
}

/** 
# Put mapping builder

A request builder for a [Put Mapping][docs-mapping] request.

Call [`Client.put_mapping`][Client.put_mapping] to get a `RequestBuilder` for a put mapping request.

[Client.put_mapping]: ../struct.Client.html#method.put_mapping
[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
*/
impl<'a, TDocument> RequestBuilder<'a, PutMappingRequestBuilder<TDocument>>
    where TDocument: DocumentType
{
    /** Set the type for the put mapping request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.into();
        self
    }

    /** Send the put mapping request. */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.req.into_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestBuilder::new(req))
            .send_raw()?
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .put_mapping::<Value>(index("test-idx"))
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/value", req.url.as_ref());
        assert_eq!(r#"{"properties":{}}"#.as_bytes().to_vec(), req.body);
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .put_mapping::<Value>(index("test-idx"))
            .ty("new-ty")
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/new-ty", req.url.as_ref());
    }
}