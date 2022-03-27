use crate::api::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::StreamRule;
use crate::id::{Id, IntoId};
use crate::meta::StreamRuleMeta;
use crate::query::UrlQueryExt;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DraftStreamRuleAdd {
    value: String,
    tag: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct DraftStreamRuleDelete {
    ids: Vec<Id>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct DraftStreamRule {
    add: Option<Vec<DraftStreamRuleAdd>>,
    delete: Option<DraftStreamRuleDelete>,
}

#[derive(Debug)]
pub struct StreamRuleBuilder<A> {
    client: TwitterApi<A>,
    url: Url,
    stream_rule: DraftStreamRule,
}
impl<A> StreamRuleBuilder<A>
where
    A: Authorization,
{
    pub fn new(client: &TwitterApi<A>, url: Url) -> Self {
        Self {
            client: client.clone(),
            url,
            stream_rule: Default::default(),
        }
    }
    pub fn dry_run(&mut self) -> &mut Self {
        self.url.append_query_val("dry_run", true);
        self
    }
    pub fn add(&mut self, value: impl ToString) -> &mut Self {
        if let Some(add) = self.stream_rule.add.as_mut() {
            add.push(DraftStreamRuleAdd {
                value: value.to_string(),
                tag: None,
            });
        } else {
            self.stream_rule.add = Some(vec![DraftStreamRuleAdd {
                value: value.to_string(),
                tag: None,
            }]);
        }
        self
    }
    pub fn add_tagged(&mut self, value: impl ToString, tag: impl ToString) -> &mut Self {
        if let Some(add) = self.stream_rule.add.as_mut() {
            add.push(DraftStreamRuleAdd {
                value: value.to_string(),
                tag: Some(tag.to_string()),
            });
        } else {
            self.stream_rule.add = Some(vec![DraftStreamRuleAdd {
                value: value.to_string(),
                tag: Some(tag.to_string()),
            }]);
        }
        self
    }
    pub fn delete_id(&mut self, id: impl IntoId) -> &mut Self {
        self.delete_ids([id]);
        self
    }
    pub fn delete_ids(&mut self, ids: impl IntoIterator<Item = impl IntoId>) -> &mut Self {
        if let Some(delete) = self.stream_rule.delete.as_mut() {
            for id in ids {
                delete.ids.push(id.into_id())
            }
        } else {
            self.stream_rule.delete = Some(DraftStreamRuleDelete {
                ids: ids.into_iter().map(|id| id.into_id()).collect(),
            });
        }
        self
    }
    pub async fn send(&self) -> ApiResult<A, Option<Vec<StreamRule>>, StreamRuleMeta> {
        self.client
            .send(
                self.client
                    .request(Method::POST, self.url.clone())
                    .json(&self.stream_rule),
            )
            .await
    }
}

impl<A> Clone for StreamRuleBuilder<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            stream_rule: self.stream_rule.clone(),
        }
    }
}
