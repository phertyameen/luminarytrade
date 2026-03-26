import { Args, Mutation, Query, Resolver, Subscription } from '@nestjs/graphql';
import { UseGuards } from '@nestjs/common';
import { OracleService } from '../../oracle/oracle.service';
import { OracleFilterInput, UpdateOracleInput } from '../types/graphql-inputs';
import { GraphqlPubSub } from '../pubsub.service';
import { ORACLE_UPDATED_EVENT } from '../graphql.constants';
import { GqlJwtAuthGuard } from '../guards/gql-jwt-auth.guard';

@Resolver('Oracle')
export class OracleResolver {
  constructor(
    private readonly oracleService: OracleService,
    private readonly pubSub: GraphqlPubSub,
  ) {}

  @Query('oracle')
  async oracle(@Args('id') id: string) {
    return this.oracleService.getSnapshotAsOracle(id);
  }

  @Query('oracles')
  async oracles(@Args('filter', { nullable: true }) filter?: OracleFilterInput) {
    const latest = await this.oracleService.getLatest();
    let rows = latest;

    if (filter?.pair) {
      rows = rows.filter((item) => item.pair === filter.pair);
    }

    const limited = rows.slice(0, filter?.limit || rows.length);
    return limited.map((row) => ({
      id: `${row.pair}-${row.timestamp?.toString?.() || Date.now()}`,
      pair: row.pair,
      price: row.price,
      decimals: row.decimals,
      timestamp: row.timestamp,
      snapshotId: row.snapshotId || '',
    }));
  }

  @UseGuards(GqlJwtAuthGuard)
  @Mutation('updateOracleData')
  async updateOracleData(@Args('input') input: UpdateOracleInput) {
    const result = await this.oracleService.updateSnapshot(input as any);
    const latest = await this.oracleService.getLatest();
    const first = latest[0];
    const payload = {
      id: result.snapshotId,
      pair: first?.pair || '',
      price: first?.price || '0',
      decimals: first?.decimals || 0,
      timestamp: first?.timestamp || new Date(),
      snapshotId: result.snapshotId,
    };

    await this.pubSub.pubSub.publish(ORACLE_UPDATED_EVENT, { oracleDataUpdated: payload });
    return payload;
  }

  @Subscription('oracleDataUpdated', {
    resolve: (payload) => payload.oracleDataUpdated,
  })
  oracleDataUpdated() {
    return this.pubSub.pubSub.asyncIterator(ORACLE_UPDATED_EVENT);
  }
}
