import { getServerSession, getToken } from '#auth';
import prisma from './../../../db/prisma';

export default defineEventHandler(async (event) => {
    const session = await getServerSession(event)
    if (!session) {
        setResponseStatus(event, 404)
        return event
    }

    const token = await getToken({ event })

    const queryParams = getRouterParams(event)
    const studySetId = queryParams.id

    if (token?.sub) {
        try {
            const studyset = await prisma.studySet.findFirst({
                where: {
                    id: +studySetId
                }
            });

            if (studyset?.userId === +token.sub) {
                setResponseStatus(event, 200)
                return event
            }
            setResponseStatus(event, 401)
            return event
        } catch (error) {
            setResponseStatus(event, 404)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})