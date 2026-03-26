import { Args, Mutation, Query, Resolver } from '@nestjs/graphql';
import { UseGuards } from '@nestjs/common';
import { SubmitterService } from '../../submitter/submitter.service';
import { CreateSubmissionInput } from '../types/graphql-inputs';
import { GqlJwtAuthGuard } from '../guards/gql-jwt-auth.guard';

@Resolver('Submission')
export class SubmissionResolver {
  constructor(private readonly submitterService: SubmitterService) {}

  @Query('submission')
  async submission(@Args('id') id: string) {
    return this.submitterService.getSubmission(id);
  }

  @Query('submissions')
  async submissions(
    @Args('limit', { nullable: true }) limit = 50,
    @Args('offset', { nullable: true }) offset = 0,
    @Args('status', { nullable: true }) status?: string,
  ) {
    const rows = await this.submitterService.listSubmissions(status as any);
    return rows.slice(offset, offset + limit);
  }

  @Query('transaction')
  async transaction(@Args('id') id: string) {
    const submission = await this.submitterService.getSubmission(id);
    return {
      id: submission.id,
      submissionId: submission.id,
      hash: submission.transactionHash,
      status: submission.status,
      createdAt: submission.createdAt,
    };
  }

  @Query('transactions')
  async transactions(
    @Args('limit', { nullable: true }) limit = 50,
    @Args('offset', { nullable: true }) offset = 0,
    @Args('status', { nullable: true }) status?: string,
  ) {
    const rows = await this.submitterService.listSubmissions(status as any);
    return rows.slice(offset, offset + limit).map((submission) => ({
      id: submission.id,
      submissionId: submission.id,
      hash: submission.transactionHash,
      status: submission.status,
      createdAt: submission.createdAt,
    }));
  }

  @UseGuards(GqlJwtAuthGuard)
  @Mutation('createSubmission')
  async createSubmission(@Args('input') input: CreateSubmissionInput) {
    return this.submitterService.createSubmission(input as any);
  }
}
