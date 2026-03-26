import { IsNotEmpty, IsObject, IsString } from 'class-validator';

export class CreateSubmissionDto {
  @IsString()
  @IsNotEmpty()
  idempotencyKey: string;

  @IsObject()
  payload: Record<string, any>;
}
