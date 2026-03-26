import { Args, Query, Resolver } from '@nestjs/graphql';
import { NotFoundException } from '@nestjs/common';
import { AuditLogService } from '../../audit/audit-log.service';
import { AuditLogFilterInput } from '../types/graphql-inputs';

@Resolver('AuditLog')
export class AuditLogResolver {
  constructor(private readonly auditLogService: AuditLogService) {}

  @Query('auditLog')
  async auditLog(@Args('id') id: string) {
    const found = await this.auditLogService.getById(id);
    if (!found) {
      throw new NotFoundException(`AuditLog with ID ${id} not found`);
    }
    return found;
  }

  @Query('auditLogs')
  async auditLogs(@Args('filter', { nullable: true }) filter?: AuditLogFilterInput) {
    if (!filter) {
      const result = await this.auditLogService.fetchAuditLogs({ limit: 50, offset: 0 } as any);
      return result.logs;
    }

    if (filter.wallet) {
      return this.auditLogService.getLogsByWallet(filter.wallet, filter.limit || 50);
    }

    if (filter.eventType) {
      return this.auditLogService.getLogsByEventType(filter.eventType as any, filter.limit || 50);
    }

    const result = await this.auditLogService.fetchAuditLogs(filter as any);
    return result.logs;
  }
}
