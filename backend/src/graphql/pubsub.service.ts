import { Injectable } from '@nestjs/common';
import { PubSub } from 'graphql-subscriptions';

@Injectable()
export class GraphqlPubSub {
  readonly pubSub = new PubSub();
}
